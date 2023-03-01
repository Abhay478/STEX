import 'dart:convert';

// ignore: unused_import
import 'package:flutter/foundation.dart'; // used when debugging

import 'package:http/http.dart' as http;

import '../models/question.dart';

// TODO: switch to flutter_dotenv?
const String backendUrl = 'http://localhost:8080';
final client = http.Client();

class CompletionResult {
  late final int id;
  late final String text;

  CompletionResult(this.id, this.text);

  // fromjson
  CompletionResult.fromJson(Map<String, dynamic> json) {
    for (final key in json.keys) {
      if (key == 'id') {
        id = json[key];
      } else {
        text = json[key];
      }
    }
  }

  // list from json array
  static List<CompletionResult> listFromJson(List<dynamic> json) {
    return json.map((e) => CompletionResult.fromJson(e)).toList();
  }
}

class SearchType {
  const SearchType(this.name, this.displayName, this.placeholder, this.route);
  final String name;
  final String displayName;
  final String placeholder;
  final String route; // used for the URL
}

Future<List<CompletionResult>> getCompletionResults(String value, SearchType type) async {
  try {
    final uri = Uri.parse('$backendUrl/auto/${type.route}').replace(queryParameters: {'q': value});
    final response = await client.get(uri);
    if (response.statusCode == 200) {
      return CompletionResult.listFromJson(jsonDecode(response.body));
    } else {
      //debugPrint('error getting completion results: ${response.statusCode}');
      return [];
    }
  } catch (e) {
    //debugPrint('error getting completion results: $e');
    return [];
  }
}

enum SortOrder { newest, oldest, mostVotes, leastVotes }

const Map<SortOrder, Map<String, String>> sortOrderMap = {
  SortOrder.newest: {'attr': 'time', 'dir': 'false'},
  SortOrder.oldest: {'attr': 'time', 'dir': 'true'},
  SortOrder.mostVotes: {'attr': 'score', 'dir': 'false'},
  SortOrder.leastVotes: {'attr': 'score', 'dir': 'true'},
};

Future<List<Post>> getUserQuestions(String username, SortOrder sortOrder) async {
  try {
    final uri = Uri.parse('$backendUrl/user/$username/questions').replace(queryParameters: sortOrderMap[sortOrder]!);
    final response = await client.get(uri);
    if (response.statusCode == 200) {
      return Post.listFromJson(jsonDecode(response.body));
    } else {
      //debugPrint('error getting user questions: ${response.statusCode}');
      return [];
    }
  } catch (e) {
    //debugPrint('error getting user questions: $e');
    return [];
  }
}

Future<List<Post>> getTaggedQuestions(String tags, SortOrder sortOrder) async {
  String tagList = tags.split(' ').map((tag) => tag == '' ? '' : '<$tag>').join();
  try {
    final uri = Uri.parse('$backendUrl/search/tags').replace(queryParameters: {
      ...sortOrderMap[sortOrder]!,
      'text': tagList,
    });
    final response = await client.get(uri);
    if (response.statusCode == 200) {
      return Post.listFromJson(jsonDecode(response.body));
    } else {
      //debugPrint('error getting tags: ${response.statusCode}');
      return [];
    }
  } catch (e) {
    //debugPrint('error getting tags: $e');
    return [];
  }
}

Future<List<Post>> searchQuestions(String searchString, SortOrder sortOrder) async {
  try {
    final uri = Uri.parse('$backendUrl/search/title').replace(queryParameters: {
      ...sortOrderMap[sortOrder]!,
      'text': searchString,
    });
    final response = await client.get(uri);
    if (response.statusCode == 200) {
      return Post.listFromJson(jsonDecode(response.body));
    } else {
      //debugPrint('error searching questions: ${response.statusCode}');
      return [];
    }
  } catch (e) {
    //debugPrint('error searching questions: $e');
    return [];
  }
}

class QuestionAndAnswers {
  late final Post question;
  late final List<Post> answers;

  QuestionAndAnswers(this.question, this.answers);

  // fromjson
  QuestionAndAnswers.fromJson(Map<String, dynamic> json) {
    question = Post.fromJson(json['q']);
    answers = Post.listFromJson(json['a']);
  }
}

Future<QuestionAndAnswers?> getQuestionAndAnswers(String id) async {
  try {
    final uri = Uri.parse('$backendUrl/qa/$id');
    final response = await client.get(uri);
    if (response.statusCode == 200) {
      final body = jsonDecode(response.body);
      return QuestionAndAnswers.fromJson(body);
    } else {
      //debugPrint('error getting question and answers: ${response.statusCode}');
      return null;
    }
  } catch (e) {
    //debugPrint('error getting question and answers: $e');
    return null;
  }
}
