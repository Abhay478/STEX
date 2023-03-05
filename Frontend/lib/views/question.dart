import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:html_editor_enhanced/html_editor.dart';
import 'package:stex_web/utils/answer_editor.dart';
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

  List<bool> isEditingAnswer = [];

  bool isAnswering = false;

  @override
  void initState() {
    super.initState();
    getQuestionAndAnswers(widget.questionId).then((qna) {
      setState(() {
        questionAndAnswers = qna;
        isEditingAnswer = List<bool>.filled(qna!.answers.length, false);
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
              onPostDeleted: () {
                if (context.mounted) {
                  // Instead of importing loggedInUser, just use the owner of the question, since
                  // we know its the same user
                  context.go('/user/${questionAndAnswers!.question.ownerUserId}');
                }
              },
            ),
            Padding(
              padding: const EdgeInsets.all(10),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text('${questionAndAnswers!.answers.length} answers: ', style: Theme.of(context).textTheme.titleLarge),
                  if (loggedInUser != null && !isAnswering)
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
                child: AnswerEditor(
                  onSubmit: (answer) async {
                    final newAnswer = await postAnswer(questionAndAnswers!.question.id!, answer);
                    if (!mounted) return false;
                    if (newAnswer == null) {
                      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                        content: Text('Failed to post answer'),
                      ));
                      return false;
                    }
                    setState(() {
                      questionAndAnswers!.answers.insert(0, newAnswer);
                      isEditingAnswer = [false, ...isEditingAnswer];
                      isAnswering = false;
                    });
                    return true;
                  },
                  onCancel: () {
                    setState(() {
                      isAnswering = false;
                    });
                  },
                )
              ),
              ...Iterable<int>.generate(questionAndAnswers!.answers.length).map((idx) {
                final answer = questionAndAnswers!.answers[idx];
                if (!isEditingAnswer[idx]) {
                  return PostCard(
                    postType: PostType.answer,
                    post: answer,
                    onEdit: () {
                      setState(() {
                        isEditingAnswer[idx] = true;
                      });
                    },
                    onPostDeleted: () {
                      setState(() {
                        questionAndAnswers!.answers.remove(answer);
                      });
                    },
                  );
                } else {
                  // TODO: answer editor
                  return Card(
                    margin: const EdgeInsets.all(10),
                    child: AnswerEditor(
                      initialText: answer.body ?? '',
                      onSubmit: (newAnswer) async {
                        final result = await updatePost(answer.id.toString(), null, null, newAnswer);
                        if (!mounted) return false;
                        if (result == null) {
                          ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                            content: Text('Failed to update answer'),
                          ));
                          return false;
                        }
                        setState(() {
                          questionAndAnswers!.answers[idx].body = newAnswer;
                          isEditingAnswer[idx] = false;
                        });
                        return true;
                      },
                      onCancel: () {
                        setState(() {
                          isEditingAnswer[idx] = false;
                        });
                      },
                    )
                  );
                }
              }),
          ]
        ),
    );
  }
}
