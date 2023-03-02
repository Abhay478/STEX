import 'package:flutter/material.dart';

import '../utils/web.dart';
import './post_list.dart';

// path: '/user/:id'

class UserPage extends StatelessWidget {
  const UserPage({super.key, required this.userId});

  final String userId;

  @override
  Widget build(BuildContext context) {
    return PostListPage(getQuestions: (ord) async => await getUserQuestions(userId, ord));
  }
}

//class UserPage extends StatefulWidget {
  //const UserPage({super.key, required this.userId});

  //final String userId;

  //@override
  //State<UserPage> createState() => _UserPageState();
//}

//class _UserPageState extends State<UserPage> {

  //List<Post>? questions;

  //@override
  //void initState() {
    //super.initState();
    //getUserQuestions(widget.userId).then((qns) {
      //setState(() {
        //questions = qns;
      //});
    //});
  //}

  //@override
  //Widget build(BuildContext context) {
    //return Scaffold(
      //appBar: buildAppBar(context),
      //body: QuestionPreviewList(questions: questions),
    //);
  //}
//}
