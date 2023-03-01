import 'package:flutter/material.dart';

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
              child: Text('${questionAndAnswers!.answers.length} answers: ', style: Theme.of(context).textTheme.titleLarge),
            ),
            const Divider(),
            ...questionAndAnswers!.answers.map((a) => PostCard(
              postType: PostType.answer,
              post: a,
            )),
          ]
        ),
    );
  }
}
