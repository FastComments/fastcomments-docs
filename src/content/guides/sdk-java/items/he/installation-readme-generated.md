### Maven

הוסף את מאגר Repsy ל‑POM של הפרויקט שלך:

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
    <!-- לקוח API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- ספריית ליבה (כוללת SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- ספריית PubSub (לאירועים בזמן אמת) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    // לקוח API
    implementation "com.fastcomments:client:3.1.1"
    
    // ספריית ליבה (כוללת SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // ספריית PubSub (לאירועים בזמן אמת)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

ספרייה זו מכילה שלושה מודולים. לקוח ה‑API שנוצר, ספריית הליבה של Java שמכילה כלי עזר שנכתבו ידנית כדי להקל על העבודה עם ה‑API, ומודול `pubsub` שהוא ספרייה למנוי על פיד של שינויים.

- [תיעוד ספריית לקוח API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית הליבה, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

עבור לקוח ה‑API, קיימות שלוש מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות את מפתח ה‑API שלך, ו‑`PublicApi` מכילה שיטות שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.

`ModerationApi` מספקת חבילה נרחבת של API למודרציה בזמן אמת ומהירה. כל שיטה ב‑`ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או באמצעות קוביית סשן של FastComments.com.