Dodajte FastComments SDK u datoteku `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Uverite se da imate Repsy repozitorijum u `settings.gradle.kts` datoteci vašeg projekta:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // ostali repozitorijumi...
    }
}
```