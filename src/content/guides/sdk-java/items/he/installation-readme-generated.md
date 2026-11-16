### Maven

הוסיפו את מאגר Repsy לקובץ ה-POM של הפרויקט שלכם:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

לאחר מכן הוסיפו את התלויות שאתם צריכים:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

הוסיפו את מאגר Repsy לקובץ ה-build.gradle שלכם:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### תוכן הספרייה

ספרייה זו מכילה שלושה מודולים: את לקוח ה-API שנוצר באופן אוטומטי, את ספריית הליבה של Java המכילה כלים שנכתבו באופן ידני
כדי להקל על העבודה עם ה-API, ואת מודול `pubsub` שהוא ספרייה להרשמה לעדכוני שינויים (change feeds).

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית הליבה, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs ציבוריים מול APIs מאובטחים

בלקוח ה-API ישנן שלוש מחלקות: `DefaultApi`, `PublicApi`, ו-`ModerationApi`. ה-`DefaultApi` מכיל שיטות הדורשות את מפתח ה-API שלכם, וה-`PublicApi` מכיל שיטות
שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/כו' ללא אימות.

ה-`ModerationApi` מספק חבילה נרחבת של APIs לניהול תגובות (moderation) מהיר ובזמן אמת. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לבצע אימות באמצעות SSO או באמצעות עוגיית סשן של FastComments.com.