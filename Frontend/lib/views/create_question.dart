import 'package:flutter/material.dart';
import 'package:stex_web/utils/app_bar.dart';

class CreateQuestionPage extends StatelessWidget {
  const CreateQuestionPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: const Center(
        child: Text('Create Question'),
      ),
    );
  }
}
