# MCMU3
第三代MCMU：Minecraft联机工具，Rust语言设计核心，Flutter框架设计GUI

### MCMU和BDS有什么区别？
MCMU是一个转发服务器，关键在于**转发玩家的数据**以达到联机目的，并不在云端运行Minecraft服务器。BDS则是**处理玩家的数据**并提供Minecraft相关功能和运算能力的服务器。

### MCMU和多玩联机盒子这类软件的区别？
MCMU的预期功能大于其他这类软件的功能，MCMU不仅提供转发服务，而且还将支持在本地运行Minecraft服务器（PocketMine-MP或BDS），利用了玩家的设备运算能力，具有“微服务器”的功能。

### 微服务器又是什么？
在本地运行服务器，并通过增量更新使存档与其他玩家保持一致，这样有权限访问该存档的玩家在任何时候都可以进入该世界游玩，而不必等某一个人开服。
