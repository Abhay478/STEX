import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:html_editor_enhanced/html_editor.dart';

import 'web.dart';

/// submit updated question
/// on (tag) error, return false
typedef OnSubmit = Future<bool> Function(String title, String tagList, String body);

class QuestionEditor extends StatefulWidget {
  const QuestionEditor({
    super.key,
    required this.onSubmit,
    this.onCancel,
    this.initialTitle = '',
    this.initialTags = '',
    this.initialBody = '',
  });

  final String initialTitle;
  final String initialTags;
  final String initialBody;

  final OnSubmit onSubmit;
  final VoidCallback? onCancel;

  @override
  State<QuestionEditor> createState() => _QuestionEditorState();
}

class _QuestionEditorState extends State<QuestionEditor> {
  late String title;
  bool tagError = false;
  final TextEditingController _tagsController = TextEditingController();
  final htmlController = HtmlEditorController();

  @override
  void initState() {
    super.initState();
    title = widget.initialTitle;
    _tagsController.text = widget.initialTags;
  }

  @override
  void dispose() {
    _tagsController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const SizedBox(height: 10),
        TextField(
          decoration: const InputDecoration(
            labelText: 'Question Title',
            hintText: 'Enter a short summary of your question',
          ),
          onChanged: (value) {
            setState(() {
              title = value;
            });
          },
        ),
        // TODO: Tag autocomplete
        TextField(
          decoration: InputDecoration(
            labelText: 'Tags',
            hintText: 'Enter a space separated list of tags',
            errorText: tagError ? 'Invalid tag(s) found' : null,
          ),
          controller: _tagsController,
        ),
        const SizedBox(height: 10),
        HtmlEditor(
          controller: htmlController,
          htmlEditorOptions: HtmlEditorOptions(
            initialText: widget.initialBody,
            hint: 'Enter a detailed explanation of your question',
            autoAdjustHeight: true,
          ),
        ),
        const SizedBox(height: 10),
        Row(
          children: [
            ElevatedButton(
              onPressed: () async {
                String tagList = _tagsController.text.split(' ').map((tag) => tag == '' ? '' : '<$tag>').join();
                final htmlText = await htmlController.getText();
                final result = await widget.onSubmit(title, tagList, htmlText);
                if (!result) {
                  setState(() {
                    tagError = true;
                  });
                }
              },
              child: const Text('Submit'),
            ),
          ],
        ),
      ]
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
