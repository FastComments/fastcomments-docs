अपने ऐप की `build.gradle.kts` फ़ाइल में FastComments SDK जोड़ें:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

सुनिश्चित करें कि आपके प्रोजेक्ट की `settings.gradle.kts` में Repsy रिपॉज़िटरी मौजूद है:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // अन्य रिपॉज़िटरी...
    }
}
```