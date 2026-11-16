הוסיפו את ה-FastComments SDK לקובץ `build.gradle.kts` של האפליקציה שלכם:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

ודאו שיש לכם את מאגר Repsy ב-`settings.gradle.kts` של הפרויקט:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```