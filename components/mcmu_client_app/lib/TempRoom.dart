import 'package:flutter/material.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'HomePage.dart';

class TempRoomPagej extends StatefulWidget {
  const TempRoomPagej({Key? key}) : super(key: key);
  @override
  _TempRoomPagejState createState() => _TempRoomPagejState();
}

class _TempRoomPagejState extends State {
  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Scaffold(
        body: CustomScrollView(
          physics: ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            SliverAppBar(
              expandedHeight: 256,
              backgroundColor: Color.fromARGB(255, 117, 194, 121),
              flexibleSpace: FlexibleSpaceBar(
                title: Text('房间'),
                collapseMode: CollapseMode.pin,
              ),
            ),
            /* FloatingActionButton(
              onPressed: null,
            ), */
          ],
        ),
      ),
    );
  }
}
