import 'package:flutter/material.dart';

import '../utils/app_bar.dart';
import '../utils/post_list.dart';
import '../utils/web.dart';
import './post_list.dart';

// path: '/user/:id'

class UserPage extends StatelessWidget {
  const UserPage({super.key, required this.userId});

  final String userId;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: ListView(
        children: [
          const SizedBox(height: 20),
          Text('Questions', style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 20),
          SizedBox(
            height: 500,
            child: PostList(getQuestions: (ord) async => await getUserQuestions(userId, ord))
          ),
          const SizedBox(height: 20),
          Text('Answers', style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 20),
          SizedBox(
            height: 500,
            child: PostList(getQuestions: (ord) async => await getUserAnswers(userId, ord))
          ),
        ],
      ),
    );
  }
}
/*
class UserPage extends StatelessWidget {
  const UserPage({super.key, required this.userId});

  final String userId;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Column(
            children: [
              const SizedBox(height: 20),
              const Text('Questions'),
              const SizedBox(height: 20),
              // TODO: question list
              PostList(getQuestions: (ord) async => await getUserQuestions(userId, ord)),
            ]
          ),
          Column(
            children: [
              const SizedBox(height: 20),
              const Text('Answers'),
              const SizedBox(height: 20),
              // TODO: answer list
              PostList(getQuestions: (ord) async => await getUserAnswers(userId, ord)),
            ]
          ),
        ]
      )
    );
  }
}
*/
