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
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: QuestionEditor(
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
              },
            ),
          )
        ]
      )
    );
  }
}
