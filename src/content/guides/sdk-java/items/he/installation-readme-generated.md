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

לאחר מכן הוסיפו את יחסי התלות הדרושים:

```xml
<dependencies>
    <!-- לקוח API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- ספריית הליבה (כולל SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- ספריית PubSub (עבור אירועים חיים) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    // לקוח API
    implementation "com.fastcomments:client:3.1.1"
    
    // ספריית הליבה (כולל SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // ספריית PubSub (עבור אירועים חיים)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### תוכן הספרייה

ספרייה זו מכילה שלושה מודולים: לקוח ה-API שנוצר אוטומטית, ספריית הליבה של Java המכילה כלים שנכתבו באופן ידני
כדי להקל על העבודה עם ה-API, ומודול ה-`pubsub` שהוא ספרייה להרשמה לעדכוני שינויים (change feeds).

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית הליבה, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs ציבוריים מול APIs מאובטחים

בלקוח ה-API קיימות שלוש מחלקות: `DefaultApi`, `PublicApi`, ו-`ModerationApi`. ה-`DefaultApi` מכיל שיטות הדורשות את מפתח ה-API שלכם, וה-`PublicApi` מכיל שיטות
שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/כדומה ללא אימות.

ה-`ModerationApi` מספק חבילה נרחבת של APIs לניהול (moderation) חי ומהיר. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לבצע אימות באמצעות SSO או באמצעות עוגיית סשן של FastComments.com.