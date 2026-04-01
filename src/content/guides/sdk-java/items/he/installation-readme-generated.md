### Maven

הוסף את מאגר Repsy ל-POM של הפרויקט שלך:

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
        <version>1.3.2</version>
    </dependency>
    
    <!-- ספריית Core (כוללת SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- ספריית PubSub (לאירועים חיים) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    implementation "com.fastcomments:client:1.3.2"
    
    // ספריית Core (כוללת SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // ספריית PubSub (לאירועים חיים)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

ספרייה זו מכילה שלושה מודולים. לקוח ה-API שנוצר, ספריית ה-Java המרכזית שמכילה כלי עזר שנכתבו ביד כדי להקל על העבודה עם ה-API, ומודול `pubsub` — ספרייה למנוי לזרמי שינויים.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית Core, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

לגבי לקוח ה-API, יש שתי מחלקות, `DefaultApi` ו-`PublicApi`. המחלקה `DefaultApi` מכילה שיטות שדורשות את מפתח ה-API שלך, ו-`PublicApi` מכילה קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.