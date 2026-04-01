### Maven

Προσθέστε το αποθετήριο Repsy στο POM του έργου σας:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Στη συνέχεια προσθέστε τις εξαρτήσεις που χρειάζεστε:

```xml
<dependencies>
    <!-- Πελάτης API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Βασική Βιβλιοθήκη (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για live συμβάντα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:1.3.2"
    
    // Βασική Βιβλιοθήκη (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Βιβλιοθήκη PubSub (για live συμβάντα)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρία modules. Ο παραγόμενος πελάτης API, η βασική Java βιβλιοθήκη που περιέχει χειρογραμμένα βοηθητικά εργαλεία για να διευκολύνουν τη δουλειά με το API, και το module `pubsub` το οποίο είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Περιλαμβάνει Παραδείγματα SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια έναντι Προστατευμένων API

Για τον πελάτη API, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το API key σας, ενώ η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από έναν browser/συσκευή κινητού/κ.λπ. χωρίς έλεγχο ταυτότητας.