### Format Example

The Scheduled Comments CSV looks like this:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              |Delay|Delay Unit|
|---|---------|-------------|---|-------------|------|-------------------------------------|-----|----------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               |1    |seconds   |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         |3    |seconds   |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     |4    |seconds   |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?|20   |seconds   |


### Format Details

The Scheduled Comments CSV file supports the following columns:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Delay
- Delay Unit

The following columns are **required**:

- ID
- URL ID
- Name
- Comment
- Delay
- Delay Unit

You'll need the Parent ID column if you want to supported automated nested replies.

### How The Format Works

The import format works like this:

1. You define a row in the CSV for every comment you want posted.
2. The comment is posted after the delay specified.
   1. The delay is separated into the raw numeric value, and the unit you want to use.
   2. For example, you might have a delay of "5" and a unit of "seconds". You can even extend this to months, by having a delay of "3" and a unit of "months".
   3. Delays are relative to when you hit "play" in the UI, after the import is done - **not when the import starts**.
3. You must define an ID. You can just use incrementing identifiers like 1, 2, 3, 4, 5.
4. If you want to use nested replies, you just set the `Parent ID` column value to the `ID` of another comment.
5. The `Comment` field supports the same syntax as FastComments supports in its comment widget for styling text and adding images.
6. The `Avatar` field, if used, must be a publicly accessible image. It will be copied to and served from our CDN.
7. You can delete all the comments after the replay, or if the replay is stopped.
8. Deletion happens live, so you can reuse the same scheduled import over and over.

Supported values for the `Delay Unit` column are:

- seconds
- minutes
- hours
- days
- weeks
- months
- years

### An Example

[An example CSV file is here](/csv/fastcomments-scheduled-comments-example.csv). 
