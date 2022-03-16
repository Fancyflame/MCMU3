// ignore: file_names
import 'package:flutter/material.dart';
import 'package:mcmu_flutter/HomePage.dart';

class Initial extends StatelessWidget {
  const Initial({Key? key}) : super(key: key);
  @override
  Widget build(BuildContext context) {
    Future.delayed(const Duration(milliseconds: 1000)).then((e) {
      Navigator.pushAndRemoveUntil(
          context,
          MaterialPageRoute(builder: (context) => const HomePage()),
          ((route) => route == null));
    });

    return const MaterialApp(
      title: 'MCMUII-test',
      home: Scaffold(
        body: Center(
          child: Text('初始化页面'),
        ),
      ),
    );
  }
}
