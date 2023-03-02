import 'package:flutter/material.dart';
import 'package:html_editor_enhanced/html_editor.dart';
import 'package:stex_web/utils/app_bar.dart';
import 'package:stex_web/utils/web.dart';

class CreateQuestionPage extends StatefulWidget {
  const CreateQuestionPage({super.key});

  @override
  State<CreateQuestionPage> createState() => _CreateQuestionPageState();
}

class _CreateQuestionPageState extends State<CreateQuestionPage> {

  String title = "";
  //String tags = "";

  //final GlobalKey _key = GlobalKey();
  //final FocusNode _focusNode = FocusNode();
  final TextEditingController _tagsController = TextEditingController();

  final htmlController = HtmlEditorController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body:
      Column(
        children: [
          const SizedBox(height: 10),
          Align(
            alignment: Alignment.centerLeft,
            child: Text('New question', style: Theme.of(context).textTheme.titleLarge)
          ),
          const SizedBox(height: 10),
          TextField(
            decoration: const InputDecoration(
              labelText: 'Question Title',
              hintText: 'Enter a short summary of your problem',
            ),
            onChanged: (value) {
              setState(() {
                title = value;
              });
            },
          ),
          TextField(
            decoration: const InputDecoration(
              labelText: 'Tags',
              hintText: 'Enter a space separated list of tags',
            ),
            controller: _tagsController,
            onChanged: (value) {
              //setState(() {
                //tags = value;
              //});
            },
          ),
          const SizedBox(height: 10),
          HtmlEditor(
            controller: htmlController,
            htmlEditorOptions: const HtmlEditorOptions(
              initialText: '',
              hint: 'Enter a detailed explanation of your problem',
              autoAdjustHeight: true,
            ),
          ),
          const SizedBox(height: 10),
          ElevatedButton(
            onPressed: () async {
              String tagList = _tagsController.text.split(' ').map((tag) => tag == '' ? '' : '<$tag>').join();
              final html = await htmlController.getText();
              debugPrint({
                'title': title,
                'tags': tagList,
                'body': html,
              }.toString());
              postQuestion(title, tagList, html);
              // TODO: Submit question
            },
            child: const Text('Submit'),
          ),
        ]
      )
    );
  }
}

/*
          RawAutocomplete<CompletionResult>(
            key: _key,
            focusNode: _focusNode,
            textEditingController: _tagsController,
            displayStringForOption: (option) => option.text,
            optionsBuilder: (textEditingValue) async {
              final String val;
              final List<String> tagList = textEditingValue.text.split(' ');
              val = tagList[tagList.length - 1];
              if (val.length < 3) {
                return const Iterable.empty();
              }
              final List<CompletionResult> results = await getCompletionResults(val, searchTypes[0]);
              return results;
            },
            optionsViewBuilder: (context, onSelected, options) {
              return Material(
                elevation: 4.0,
                child: ListView(
                  shrinkWrap: true,
                  children: options.map((CompletionResult option) {
                    return ListTile(
                      title: Text(option.text),
                      onTap: () {
                        onSelected(option);
                      },
                    );
                  }).toList(),
                )
              );
            },
            onSelected: (CompletionResult selection) {
              final List<String> tagList = _tagsController.text.split(' ');
              debugPrint(tagList.toString());
              tagList[tagList.length - 1] = selection.text;
              debugPrint(tagList.toString());
              _tagsController.text = '${tagList.join(' ')} ';
            },
            fieldViewBuilder: (context, textEditingController, focusNode, onFieldSubmitted) {
              return TextField(
                controller: textEditingController,
                focusNode: focusNode,
                decoration: const InputDecoration(
                  labelText: 'Tags',
                  hintText: 'Enter a space separated list of tags',
                ),
                //onSubmitted: (value) {
                  //onFieldSubmitted();
                //},
              );
            },
          ),
*/
