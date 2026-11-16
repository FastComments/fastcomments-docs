הסקרים פועלים באותו אופן כמו בווידג'ט האינטרנט, ומופעלים מאותן הגדרות ווידג'ט בלוח הבקרה של FastComments שלכם (או עם `pollCreation` / `pollVoting` לעיל).

- תגובות עם סקר מציגות אותו מתחת לטקסט שלהן. הקוראים מצביעים על ידי הקשה על אפשרות, יכולים להציג או להסתיר תוצאות לפני ההצבעה, ויכולים לשנות את הצבעתם. הספירות מתעדכנות בזמן אמת כאשר אחרים מצביעים.
- סקרים יכולים להיסגר בתאריך מסוים, להסתיר תוצאות עד שתצביעו, ולהפוך את המצביעים לגלויים למנהלים או לכולם ("הצג מי הצביע").
- כאשר הצופה רשאי ליצור סקרים, שדה הקלט של התגובות ב-`FastCommentsView` מציג כפתור סקר שפותח את בונה הסקרים. שורת סרגל הכלים מופיעה עבורו גם אם מעולם לא הפעלתם את סרגל הכלים; `sdk.setCommentToolbarEnabled(false)` מסתיר אותו, כולל כפתור הסקר, ו-`setPollCreationEnabled(false)` בשדה הקלט מסיר רק את כפתור הסקר. קראו ל-`setCommentToolbarEnabled` לפני יצירת ה-`FastCommentsView`, מכיוון ששדה הקלט קורא אותו כאשר הוא מקבל את ה-SDK; עבור תצוגה שכבר על המסך, קראו במקום זאת ל-`getBottomCommentInput().setToolbarVisible(false)`. שדה הקלט של הצ'אט החי משמיט אותו, כמו בצ'אט החי באינטרנט.

כדי לפרסם סקר מממשק המשתמש שלכם, העבירו `CommentPollInput` ל-`postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: an ISO 8601 close date, PollPrivacy value, and results hidden until you vote.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` ו-`sdk.getPollVoters(commentId, page, search, callback)` מכסים הצבעה ואת רשימת המצביעים. צבעי הסקר מפורטים ב-THEMING.md.