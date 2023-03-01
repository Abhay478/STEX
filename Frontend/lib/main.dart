import 'package:flutter/material.dart';
import 'package:flutter_web_plugins/url_strategy.dart';
import 'package:go_router/go_router.dart';

import 'models/user.dart';
import 'views/search_post.dart';
import 'views/search_tags.dart';
import 'views/user.dart';
import 'views/create_question.dart';
import 'views/homepage.dart';
import 'views/search.dart';
import 'views/question.dart';

final _router = GoRouter(
  routes: [
    GoRoute(
      path: '/',
      builder: (context, state) => const HomePage(),
    ),
    //GoRoute(
      //path: '/login',
      //builder: (context, state) => const LoginPage(),
    //),
    GoRoute(
      path: '/create_question',
      builder: (context, state) => const CreateQuestionPage(),
    ),
    GoRoute(
      path: '/search',
      builder: (context, state) => const SearchPage(),
    ),
    GoRoute(
      path: '/question/:id',
      builder: (context, state) => QuestionPage(questionId: state.params['id']!)
    ),
    GoRoute(
      path: '/user/:id',
      builder: (context, state) => UserPage(userId: state.params['id']!)
    ),
    GoRoute(
      path: '/search/tags',
      builder: (context, state) => SearchTagsPage(tags: state.queryParams['q'] ?? '')
    ),
    GoRoute(
      path: '/search/posts',
      builder: (context, state) => SearchPostsPage(searchString: state.queryParams['q'] ?? '')
    ),
  ]
);

void main() async {
  //usePathUrlStrategy();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      debugShowCheckedModeBanner: false,
      routerConfig: _router,
      // dark theme
      theme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.orange,
        useMaterial3: true,
      )
    );
  }
}
