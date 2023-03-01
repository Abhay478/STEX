import 'package:flutter/material.dart';

import './post_list.dart';
import '../utils/web.dart';

// path: '/search/tags?q=..'

class SearchTagsPage extends StatelessWidget {
  const SearchTagsPage({super.key, required this.tags});

  final String tags;

  @override
  Widget build(BuildContext context) {
    return PostListPage(getQuestions: (ord) async => await getTaggedQuestions(tags, ord));
  }
}
