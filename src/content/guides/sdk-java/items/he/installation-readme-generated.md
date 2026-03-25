### Maven

הוסף את המאגר Repsy ל-POM של הפרויקט שלך:

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
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

הוסף את המאגר Repsy לקובץ build.gradle שלך:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### תכולת הספרייה

ספרייה זו מכילה שלושה מודולים. לקוח ה-API שנוצר, ספריית ה-Java הליבתית שמכילה כלים שנכתבו ידנית כדי להקל על העבודה עם ה-API, ומודול ה-`pubsub` שהוא ספרייה למנויים על זרמי שינויים.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [תיעוד ספריית ה-Core, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [תיעוד ספריית PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### ממשקי API ציבוריים לעומת מאובטחים

עבור לקוח ה-API קיימות שתי מחלקות, `DefaultApi` ו-`PublicApi`. ה-`DefaultApi` מכיל שיטות שדורשות את מפתח ה-API שלך, ו-`PublicApi` מכיל קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.