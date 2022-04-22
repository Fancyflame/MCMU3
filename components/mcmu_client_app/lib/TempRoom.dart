import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'HomePage.dart';
import 'package:flutter_speed_dial/flutter_speed_dial.dart';

class TempRoomPagej extends StatefulWidget {
  const TempRoomPagej({Key? key}) : super(key: key);
  @override
  _TempRoomPagejState createState() => _TempRoomPagejState();
}

class _TempRoomPagejState extends State {
  @override
  void showOverlay() {
    final renderbox = context.findRenderObject() as RenderBox;
    final size = renderbox.size;
    OverlayEntry overlayEntry = OverlayEntry(
      builder: (context) {
        return Material(
          child: Positioned(
            width: size.width,
            child: Row(
              children: [
                Container(
                  child: TextField(),
                  width: 320,
                  height: 24,
                ),
              ],
            ),
          ),
        );
      },
    );
    Overlay.of(context)?.insert(overlayEntry);
  }

  Widget build(BuildContext context) {
    return MaterialApp(
      builder: (context, child) {
        return FlutterSmartDialog(
          child: child,
        );
      },
      home: Scaffold(
        floatingActionButton: SpeedDial(
          backgroundColor: const Color.fromARGB(255, 117, 194, 121),
          animatedIcon: AnimatedIcons.add_event,
          childMargin: const EdgeInsets.symmetric(vertical: 16),
          spacing: 8,
          children: [
            SpeedDialChild(
                child: const Icon(Icons.domain_rounded),
                foregroundColor: const Color.fromARGB(255, 117, 194, 121),
                label: '创建房间'),
            SpeedDialChild(
              child: const Icon(Icons.vpn_key_rounded),
              foregroundColor: const Color.fromARGB(255, 117, 194, 121),
              label: '加入房间',
              onTap: () {
                showOverlay();
              },
            )
          ],
        ),
        body: const CustomScrollView(
          physics: ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            SliverAppBar(
              floating: true,
              snap: true,
              automaticallyImplyLeading: true,
              expandedHeight: 256,
              backgroundColor: Color.fromARGB(255, 117, 194, 121),
              flexibleSpace: FlexibleSpaceBar(
                title: Text('房间'),
                collapseMode: CollapseMode.pin,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
