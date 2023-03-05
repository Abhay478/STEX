import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/utils/globals.dart';
import 'package:stex_web/utils/web.dart';

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

typedef OnPostDeleted = void Function();

class PostCard extends StatelessWidget {
  const PostCard({super.key, required this.postType, required this.post, this.isAcceptedAnswer = false, this.onPostDeleted});

  final PostType postType;
  final Post post;
  final bool isAcceptedAnswer;
  final OnPostDeleted? onPostDeleted;

  @override
  Widget build(BuildContext context) {
    //return const Placeholder();
    String score = post.score?.toString() ?? '';
    String title = '';
    String tags  = '';
    if (postType == PostType.question) {
      title = post.title ?? '';
      tags = post.tags ?? '';
    }
    return Card(
      margin: const EdgeInsets.all(10),
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: Text(score, style: Theme.of(context).textTheme.headlineSmall),
              title: Text(title, style: Theme.of(context).textTheme.titleLarge),
              subtitle: Text(tags, style: Theme.of(context).textTheme.titleSmall),
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
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                if (post.ownerUserId == loggedInUser?.id)
                  ...[
                    IconButton(
                      icon: const Icon(Icons.edit),
                      onPressed: () => {
                        if (postType == PostType.question) {
                          context.push('/question/${post.id}/edit')
                        }
                      }
                    ),
                    const SizedBox(width: 10),
                    IconButton(
                      icon: const Icon(Icons.delete),
                      onPressed: () async {
                        final confirmed = await showDialog<bool>(
                          context: context,
                          builder: (context) => AlertDialog(
                            title: const Text('Delete post'),
                            content: const Text('Are you sure you want to delete this post?'),
                            actions: [
                              TextButton(
                                onPressed: () => Navigator.of(context).pop(true),
                                style: ButtonStyle(
                                  backgroundColor: MaterialStateProperty.all(Colors.green),
                                  foregroundColor: MaterialStateProperty.all(Colors.black),
                                ),
                                child: const Text('Yes'),
                              ),
                              TextButton(
                                onPressed: () => Navigator.of(context).pop(false),
                                style: ButtonStyle(
                                  backgroundColor: MaterialStateProperty.all(Colors.red),
                                  foregroundColor: MaterialStateProperty.all(Colors.black),
                                ),
                                child: const Text('No'),
                              ),
                            ],
                          ),
                        );
                        if (confirmed == true) {
                          await deletePost(post.id!);
                        }
                        onPostDeleted?.call();
                      }
                    ),
                    const SizedBox(width: 10),
                  ],
                AuthorCard(postType: postType, post: post),
                const SizedBox(width: 10),
              ],
            ),
          ],
        ),
      )
    );
  }
}
