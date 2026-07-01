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

### תכולת הספרייה

הספרייה הזו מכילה שלושה מודלים. לקוח ה‑API שנוצר, ספריית הקור Java המרכזית הכוללת utilities שנכתבות ידנית כדי להקל על העבודה עם ה‑API,
ומודול `pubsub` שהוא ספרייה למנוי על פיד של שינויים.

- [תיעוד ספריית לקוח ה‑API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד הספרייה המרכזית, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API ציבוריים לעומת מאובטחים

לעבור על לקוח ה‑API, יש שלוש מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות את מפתח ה‑API שלך, ו‑`PublicApi` מכילה שיטות  
שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.

ה‑`ModerationApi` מספקת חבילה רחבה של API מודרציה בזמן אמת ומהירה. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או קוקיית סשן של FastComments.com.