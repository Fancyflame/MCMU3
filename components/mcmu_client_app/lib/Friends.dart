import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mcmu_flutter/ListData.dart';
import 'package:mcmu_flutter/main.dart';

class Friends extends StatefulWidget {
  const Friends({Key? key}) : super(key: key);
  @override
  _FriendsState createState() => _FriendsState();
}

class _FriendsState extends State<Friends> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: CustomScrollView(
          physics: const ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[getDatafromListForFriends(listData)],
          //好友功能：查看用户资料，删除好友，查看粉丝(pending)
        ),
      ),
    );
  }
}
