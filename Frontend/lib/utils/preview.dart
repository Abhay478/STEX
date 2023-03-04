import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../models/question.dart';

class _QuestionPreviewCard extends StatelessWidget {
  const _QuestionPreviewCard({required this.question});

  final Post question;

  @override
  Widget build(BuildContext context) {
    final String title = question.title ?? '';
    final String tags = question.tags ?? '';
    final String score = (question.score ?? '').toString();
    return ListTile(
      leading: Text(score, style: Theme.of(context).textTheme.headlineSmall),
      title: Text(title, style: Theme.of(context).textTheme.titleLarge),
      //subtitle: Text(tags, style: Theme.of(context).textTheme.titleSmall),
      subtitle: Row(
        children: [
          Text(tags, style: Theme.of(context).textTheme.titleSmall),
          const Spacer(),
          Text(question.creationDateShort(), style: Theme.of(context).textTheme.titleSmall),
        ]
      ),
      onTap: () => context.push('/question/${question.id}'),
    );
  }
}

class QuestionPreviewList extends StatelessWidget {
  const QuestionPreviewList({super.key, required this.questions});

  final List<Post>? questions;

  @override
  Widget build(BuildContext context) {
    if (questions == null) {
      return const Text('Loading...');
    } else {
      return
      Container(
        padding: const EdgeInsets.all(10),
        child: ListView(
          children: questions!.map((question) => _QuestionPreviewCard(question: question)).toList(), 
        )
      );
    }
  }
}
