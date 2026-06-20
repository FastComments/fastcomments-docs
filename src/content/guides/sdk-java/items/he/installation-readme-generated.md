### Maven

הוסף את מאגר Repsy לקובץ POM של הפרויקט שלך:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

לאחר מכן הוסף את התלויות שאתה צריך:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

הוסף את מאגר Repsy לקובץ build.gradle שלך:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

ספרייה זו מכילה שלושה מודולים. לקוח ה-API שנוצר באופן אוטומטי, ספריית ה-core ב-Java שמכילה עזרי קוד ידניים כדי להקל על העבודה עם ה-API, ומודול `pubsub` שהוא ספרייה למנוי על זרמי שינויים.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית ה-core, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

ללקוח ה-API יש שלוש מחלקות: `DefaultApi`, `PublicApi`, ו-`ModerationApi`. ה-`DefaultApi` מכיל שיטות שדורשות את מפתח ה-API שלך, ו-`PublicApi` מכיל שיטות שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.

ה-`ModerationApi` מפעיל את לוח הבקרה של המודרטור. הוא כולל שיטות למתן מענה על תגובות (רשימה, ספירה, חיפוש, לוגים וייצוא), פעולות Moderation (הסרה/שחזור, דגל, הגדרת מצב לסקירה/דואר-זבל/אישור, הצבעות, ופתיחה/סגירה של שרשור), איסורים (איסור מכתיבת תגובות, ביטול איסור, תקצירים לפני איסור, מצב האיסור והעדפות, וספירת משתמשים מנועים), ותגים ואמון (הענקה/הסרה של תג, תגיות ידניות, קבלת/הגדרת גורם אמון, ופרופיל פנימי של משתמש). כל שיטה ב-`ModerationApi` מקבלת פרמטר `sso` כך שהקריאה יכולה להתבצע בשם מודרטור שהאומת באמצעות SSO.