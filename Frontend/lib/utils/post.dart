import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:go_router/go_router.dart';

import '../models/question.dart';

enum PostType { question, answer }

class AuthorCard extends StatelessWidget {
  const AuthorCard({super.key, required this.postType, required this.post});

  final PostType postType;
  final Post post;

  @override
  Widget build(BuildContext context) {
    final String action = (postType == PostType.question) ? 'asked' : 'answered';
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('$action ${post.creationDateLong()}'),
        const SizedBox(height: 5),
        RichText(
          text: TextSpan(
            text: 'by ',
            style: Theme.of(context).textTheme.titleSmall,
            children: [
              TextSpan(
                text: post.ownerDisplayName ?? 'unknown',
                style: const TextStyle(color: Colors.blue),
                recognizer: TapGestureRecognizer()..onTap = () => context.push('/user/${post.ownerUserId}'),
              ),
            ]
          )
        ),
        const SizedBox(height: 5),
      ]
    );
  }
}

class PostCard extends StatelessWidget {
  const PostCard({super.key, required this.postType, required this.post, this.isAcceptedAnswer = false});

  final PostType postType;
  final Post post;

  final bool isAcceptedAnswer;

  @override
  Widget build(BuildContext context) {
    //return const Placeholder();
    return Card(
      margin: const EdgeInsets.all(10),
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: Text(post.score?.toString() ?? '', style: Theme.of(context).textTheme.headlineSmall),
              title: Text(post.title ?? '', style: Theme.of(context).textTheme.titleLarge),
              subtitle: Text(post.tags ?? '', style: Theme.of(context).textTheme.titleSmall),
              onTap: () => context.push('/question/${post.id}'),
            ),
            Padding(
              padding: const EdgeInsets.all(10),
              child: Html(data: post.body ?? '', style: {
                'body': Style(
                  fontSize: const FontSize(16),
                ),
                'pre': Style(
                  padding: const EdgeInsets.only(left: 40),
                ),
              }),
            ),
            Align(
              alignment: Alignment.centerRight,
              child: AuthorCard(postType: postType, post: post),
            ),
          ],
        ),
      )
    );
  }
}
