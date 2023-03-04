import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../utils/question_editor.dart';
import '../utils/app_bar.dart';
import '../utils/web.dart';

class CreateQuestionPage extends StatelessWidget {
  const CreateQuestionPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Column(
        children: [
          const SizedBox(height: 10),
          Align(
            alignment: Alignment.centerLeft,
            child: Text('New question', style: Theme.of(context).textTheme.titleLarge)
          ),
          const SizedBox(height: 10),
          QuestionEditor(
            onSubmit: (title, tagList, htmlText) async {
              final post = await postQuestion(title, tagList, htmlText);
              if (post == false) {
                // tag error
                return false;
              }
              if (context.mounted) {
                if (post.runtimeType == int) {
                  // success
                  context.push('/question/$post');
                } else {
                  ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Error posting question')));
                }
              }
              return true;
            }, // TODO
          )
        ]
      )
    );
  }
}

/*
class CreateQuestionPage extends StatefulWidget {
  const CreateQuestionPage({super.key});

  @override
  State<CreateQuestionPage> createState() => _CreateQuestionPageState();
}

class _CreateQuestionPageState extends State<CreateQuestionPage> {

  String title = "";

  bool tagError = false;

  //final GlobalKey _key = GlobalKey();
  //final FocusNode _focusNode = FocusNode();
  final TextEditingController _tagsController = TextEditingController();

  final htmlController = HtmlEditorController();

  @override
  void dispose() {
    _tagsController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Column(
        children: [
          const SizedBox(height: 10),
          Align(
            alignment: Alignment.centerLeft,
            child: Text('New question', style: Theme.of(context).textTheme.titleLarge)
          ),
          const SizedBox(height: 10),
          TextField(
            decoration: const InputDecoration(
              labelText: 'Question Title',
              hintText: 'Enter a short summary of your question',
            ),
            onChanged: (value) {
              setState(() {
                title = value;
              });
            },
          ),
          // TODO: Tag autocomplete
          TextField(
            decoration: InputDecoration(
              labelText: 'Tags',
              hintText: 'Enter a space separated list of tags',
              errorText: tagError ? 'Invalid tag(s) found' : null,
            ),
            controller: _tagsController,
          ),
          const SizedBox(height: 10),
          HtmlEditor(
            controller: htmlController,
            htmlEditorOptions: const HtmlEditorOptions(
              initialText: '',
              hint: 'Enter a detailed explanation of your question',
              autoAdjustHeight: true,
            ),
          ),
          const SizedBox(height: 10),
          ElevatedButton(
            onPressed: () async {
              String tagList = _tagsController.text.split(' ').map((tag) => tag == '' ? '' : '<$tag>').join();
              final htmlText = await htmlController.getText();
              final post = await postQuestion(title, tagList, htmlText);
              if (context.mounted) {
                if (post.runtimeType == int) {
                  context.push('/question/$post');
                } else if (post == false) {
                  setState(() {
                    tagError = true;
                  });
                } else {
                  ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Error posting question')));
                }
              }
            },
            child: const Text('Submit'),
          ),
        ]
      )
    );
  }
}
*/
