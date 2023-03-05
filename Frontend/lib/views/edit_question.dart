import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/models/question.dart';

import '../utils/app_bar.dart';
import '../utils/question_editor.dart';
import '../utils/web.dart';

class EditQuestionPage extends StatefulWidget {
  const EditQuestionPage({super.key, required this.questionId});

  final String questionId;

  @override
  State<EditQuestionPage> createState() => _EditQuestionPageState();
}

String splitJoinTags(String tags) {
  // convert from "<tag1><tag2><tag3>" to "tag1 tag2 tag3"
  return tags
      .split('>')
      .map((e) => e.isNotEmpty ? e.substring(1) : '')
      .where((e) => e.isNotEmpty)
      .join(' ');
}

class _EditQuestionPageState extends State<EditQuestionPage> {

  Post? post;

  @override
  void initState() {
    super.initState();
    getQuestionAndAnswers(widget.questionId).then((val) {
      if (!context.mounted) {
        return;
      }
      if (val == null) {
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Error loading question to edit')));
        return;
      }
      setState(() {
        post = val.question;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: (post == null) ? const Center(child: CircularProgressIndicator()) :
      Column(
        children: [
          const SizedBox(height: 10),
          Align(
            alignment: Alignment.centerLeft,
            child: Text('Edit question', style: Theme.of(context).textTheme.titleLarge)
          ),
          const SizedBox(height: 10),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: QuestionEditor(
              initialTitle: post!.title ?? '',
              initialTags: splitJoinTags(post!.tags ?? ''),
              initialBody: post!.body ?? '',
              onSubmit: (title, tagList, htmlText) async {
                final res = await updatePost(widget.questionId, title, tagList, htmlText);
                if (res == false) {
                  // tag error
                  return false;
                }
                if (context.mounted) {
                  if (res == true) {
                    // success
                    context.push('/question/${widget.questionId}');
                  } else {
                    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Error updating question')));
                  }
                }
                return true;
              },
              onCancel: () {
                if (context.canPop()) {
                  // return to previous page
                  context.pop();
                } else {
                  // return to question page
                  context.push('/question/${widget.questionId}');
                }
              },
            ),
          )
        ]
      )
    );
  }
}
