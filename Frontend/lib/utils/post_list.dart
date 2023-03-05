import 'package:flutter/material.dart';

import '../utils/preview.dart';
import '../utils/web.dart';
import '../utils/app_bar.dart';
import '../models/question.dart';

// path: '/user/:id'

typedef GetQuestionsCallback = Future<List<Post>> Function(SortOrder);

class PostList extends StatefulWidget {
  const PostList({super.key, required this.getQuestions});

  final GetQuestionsCallback getQuestions;

  @override
  State<PostList> createState() => _PostListState();
}

class _PostListState extends State<PostList> {

  List<Post>? questions;
  SortOrder sortOrder = SortOrder.mostVotes;

  @override
  void initState() {
    super.initState();
    widget.getQuestions(sortOrder).then((qns) {
      setState(() {
        questions = qns;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            const Text('Sort by: '),
            DropdownButton<SortOrder>(
              value: sortOrder,
              onChanged: (newSortOrder) {
                if (newSortOrder != null) {
                  setState(() {
                    sortOrder = newSortOrder;
                  });
                  widget.getQuestions(sortOrder).then((qns) {
                    setState(() {
                      questions = qns;
                    });
                  });
                }
              },
              items: const [
                DropdownMenuItem<SortOrder>(
                  value: SortOrder.newest,
                  child: Text('Newest'),
                ),
                DropdownMenuItem<SortOrder>(
                  value: SortOrder.oldest,
                  child: Text('Oldest'),
                ),
                DropdownMenuItem<SortOrder>(
                  value: SortOrder.mostVotes,
                  child: Text('Most Votes'),
                ),
                DropdownMenuItem<SortOrder>(
                  value: SortOrder.leastVotes,
                  child: Text('Least Votes'),
                ),
              ],
            ),
          ],
        ),
        Expanded(
          child: QuestionPreviewList(questions: questions),
        )
      ]
    );
  }
}
