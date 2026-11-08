### Maven

अपने प्रोजेक्ट के POM में Repsy रिपॉजिटरी जोड़ें:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

फिर आवश्यक डिपेंडेंसीज़ जोड़ें:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (SSO शामिल है) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (लाइव इवेंट्स के लिए) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

अपनी build.gradle फ़ाइल में Repsy रिपॉजिटरी जोड़ें:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (SSO शामिल है)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (लाइव इवेंट्स के लिए)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### लाइब्रेरी की सामग्री

इस लाइब्रेरी में तीन मॉड्यूल हैं। जेनरेट किया गया API क्लाइंट, कोर Java लाइब्रेरी जिसमें API के साथ काम करना आसान बनाने के लिए मैन्युअल रूप से लिखे गए यूटिलिटीज़ हैं, और `pubsub` मॉड्यूल जो चेंज फीड की सदस्यता लेने के लिए एक लाइब्रेरी है।

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public बनाम Secured APIs

API क्लाइंट में तीन क्लास हैं: `DefaultApi`, `PublicApi`, और `ModerationApi`। `DefaultApi` में वे मेथड हैं जिनके लिए आपकी API key आवश्यक है, और `PublicApi` में वे मेथड हैं जिन्हें बिना प्रमाणीकरण के सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से कॉल किया जा सकता है।

`ModerationApi` लाइव और तेज़ मॉडरेशन APIs का एक व्यापक सूट प्रदान करता है। हर `ModerationApi` मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणीकरण कर सकता है।