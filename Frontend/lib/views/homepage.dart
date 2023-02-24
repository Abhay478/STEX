import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/utils/app_bar.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context, isHomePage: true),
      body: const Center(
        child: Text('Home Page'),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          context.push('/create_question');
        },
        tooltip: 'Add',
        child: const Icon(Icons.add),
      ),
    );
  }
}
