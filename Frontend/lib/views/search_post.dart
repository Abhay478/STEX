import 'package:flutter/material.dart';

import './post_list.dart';
import '../utils/web.dart';

// path: '/search/posts?q=..'

class SearchPostsPage extends StatelessWidget {
  const SearchPostsPage({super.key, required this.searchString});

  final String searchString;

  @override
  Widget build(BuildContext context) {
    return PostListPage(getQuestions: (ord) async => await searchQuestions(searchString, ord));
  }
}
