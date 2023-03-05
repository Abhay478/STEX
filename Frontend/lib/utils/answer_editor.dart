import 'package:flutter/material.dart';
import 'package:html_editor_enhanced/html_editor.dart';

/// returns true on success, false on failure
typedef OnSubmit = Future<bool> Function(String answer);

class AnswerEditor extends StatefulWidget {
  const AnswerEditor({
    super.key,
    this.initialText = '',
    required this.onSubmit,
    this.onCancel,
  });

  final String initialText;
  final OnSubmit onSubmit;
  final VoidCallback? onCancel; // only for editing

  @override
  State<AnswerEditor> createState() => _AnswerEditorState();
}

class _AnswerEditorState extends State<AnswerEditor> {

  HtmlEditorController answerController = HtmlEditorController();

  @override
  Widget build(BuildContext context) {
    //return const Placeholder();
    return Column(
      children: [
        HtmlEditor(
          controller: answerController,
          htmlEditorOptions: HtmlEditorOptions(
            initialText: widget.initialText,
            hint: 'Enter your answer',
            autoAdjustHeight: true,
          ),
        ),
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              TextButton(
                onPressed: () async {
                  final answer = await answerController.getText();
                  // ignore: use_build_context_synchronously
                  if (!context.mounted) return;
                  if (answer.isEmpty) {
                    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                      content: Text('Answer cannot be empty'),
                    ));
                    return;
                  }
                  final result = await widget.onSubmit(answer);
                  if (result) {
                    setState(() {
                      answerController.clear();
                    });
                  } else {
                    if (context.mounted) {
                      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
                          content: Text('Error submitting answer'),
                      ));
                    }
                  }
                },
                child: const Text('Submit'),
              ),
              const SizedBox(width: 10),
              TextButton(
                onPressed: () {
                  answerController.clear();
                  widget.onCancel?.call();
                },
                child: const Text("Cancel"),
              ),
            ],
          ),
        )
      ],
    );
  }
}
