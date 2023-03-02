import 'package:flutter/material.dart';
import 'package:html_editor_enhanced/html_editor.dart';
import 'package:stex_web/utils/globals.dart';

import '../utils/web.dart';
import '../utils/app_bar.dart';
import '../utils/post.dart';

// path: /question/:id

class QuestionPage extends StatefulWidget {
  const QuestionPage({super.key, required this.questionId});

  final String questionId;

  @override
  State<QuestionPage> createState() => _QuestionPageState();
}

class _QuestionPageState extends State<QuestionPage> {

  QuestionAndAnswers? questionAndAnswers;

  bool isAnswering = false;
  HtmlEditorController answerController = HtmlEditorController();

  @override
  void initState() {
    super.initState();
    getQuestionAndAnswers(widget.questionId).then((qna) {
      setState(() {
        questionAndAnswers = qna;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: (questionAndAnswers == null)
        ? const Center(child: CircularProgressIndicator())
        : ListView(
          children: [
            PostCard(
              postType: PostType.question,
              post: questionAndAnswers!.question,
            ),
            Padding(
              padding: const EdgeInsets.all(10),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text('${questionAndAnswers!.answers.length} answers: ', style: Theme.of(context).textTheme.titleLarge),
                  ElevatedButton(
                    onPressed: () {
                      if (loggedInUser == null) {
                        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                          content: Text('Login to answer questions'),
                        ));
                      } else {
                        setState(() {
                          isAnswering = true;
                        });
                      }
                    },
                    child: const Text('Add Answer'),
                  )
                ],
              ),
            ),
            const Divider(),
            if (isAnswering)
              Card(
                margin: const EdgeInsets.all(10),
                child: Column(
                  children: [
                    HtmlEditor(
                      controller: answerController,
                      htmlEditorOptions: const HtmlEditorOptions(
                        initialText: '',
                        hint: 'Enter your answer',
                        autoAdjustHeight: true,
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          TextButton(
                            onPressed: () async {
                              final answer = await answerController.getText();
                              if (answer.isEmpty) {
                                if (context.mounted) {
                                  ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                                      content: Text('Answer cannot be empty'),
                                  ));
                                }
                                return;
                              }
                              final newAnswer = await postAnswer(questionAndAnswers!.question.id!, answer);
                              setState(() {
                                questionAndAnswers!.answers.insert(0, newAnswer!);
                                isAnswering = false;
                              });
                            },
                            child: const Text('Submit'),
                          ),
                          const SizedBox(width: 10),
                          IconButton(
                            onPressed: () {
                              answerController.clear();
                              setState(() {
                                isAnswering = false;
                              });
                            },
                            icon: const Icon(Icons.delete),
                          ),
                        ],
                      ),
                    )
                  ],
                ),
              ),
            ...questionAndAnswers!.answers.map((a) => PostCard(
              postType: PostType.answer,
              post: a,
            )),
          ]
        ),
    );
  }
}
