import 'package:flutter/material.dart';
import 'package:flutter_web_plugins/url_strategy.dart';
import 'package:go_router/go_router.dart';
//import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:stex_web/utils/auth.dart';

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
    )
  ]
);

void main() async {
  usePathUrlStrategy();
  //await DotEnv().load();
  //await whoami();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      debugShowCheckedModeBanner: false,
      routerConfig: _router,
      // dark theme
      theme: ThemeData(
        brightness: Brightness.dark,
        //colorScheme: ColorScheme.fromSwatch(
          //primarySwatch: Colors.orange,
          //brightness: Brightness.dark,
        //),
        primarySwatch: Colors.orange,
      )
    );
  }
}
