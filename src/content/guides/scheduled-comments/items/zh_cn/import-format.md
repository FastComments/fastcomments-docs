### 格式示例

定时评论CSV看起来像这样:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### 格式详情

定时评论CSV文件支持以下列:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

以下列是**必需的**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

如果您想支持自动化嵌套回复,您将需要Parent ID列。

### 格式如何工作

导入格式的工作方式如下:

1. 您在CSV中为要发布的每条评论定义一行。
2. 评论在指定的延迟(小时 + 分钟 + 秒)后发布。
   1. 对于手动计划的导入,延迟是相对于您在导入完成后在UI中点击"play"的时间 - **不是导入开始的时间**。
   2. 对于自动计划的导入,延迟是从页面加载时开始。
3. 您必须定义一个ID。您可以简单地使用递增的标识符,如1、2、3、4、5。
4. 如果您想使用嵌套回复,只需将`Parent ID`列值设置为另一条评论的`ID`。
5. `Comment`字段支持FastComments在其评论小部件中支持的相同语法,用于文本样式和添加图片。
6. `Avatar`字段(如果使用)必须是可公开访问的图片。它将被复制到我们的CDN并从那里提供服务。
7. 您可以在播放后删除所有评论,或者如果播放停止。
8. 删除是实时发生的,因此您可以反复重复使用相同的计划导入。

### 示例

[示例CSV文件在这里](/csv/fastcomments-scheduled-comments-example.csv)。
