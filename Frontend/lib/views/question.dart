import 'package:flutter/material.dart';
import '../utils/app_bar.dart';

// path: /question/:id

class QuestionPage extends StatelessWidget {
  const QuestionPage({super.key, required this.questionId});

  final String questionId;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: const Center(
        child: Text('View Question'),
      ),
    );
  }
}
