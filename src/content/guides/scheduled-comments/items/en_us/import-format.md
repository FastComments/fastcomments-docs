### Format Example

The Scheduled Comments CSV looks like this:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Format Details

The Scheduled Comments CSV file supports the following columns:

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

The following columns are **required**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

You'll need the Parent ID column if you want to support automated nested replies.

### How The Format Works

The import format works like this:

1. You define a row in the CSV for every comment you want posted.
2. The comment is posted after the delay specified (hours + minutes + seconds).
   1. For Manually Scheduled imports, delays are relative to when you hit "play" in the UI, after the import is done - **not when the import starts**.
   2. For Auto Schedules imports, the delay is from the time of page load.
3. You must define an ID. You can just use incrementing identifiers like 1, 2, 3, 4, 5.
4. If you want to use nested replies, you just set the `Parent ID` column value to the `ID` of another comment.
5. The `Comment` field supports the same syntax as FastComments supports in its comment widget for styling text and adding images.
6. The `Avatar` field, if used, must be a publicly accessible image. It will be copied to and served from our CDN.
7. You can delete all the comments after the replay, or if the replay is stopped.
8. Deletion happens live, so you can reuse the same scheduled import over and over.

### An Example

[An example CSV file is here](/csv/fastcomments-scheduled-comments-example.csv).
