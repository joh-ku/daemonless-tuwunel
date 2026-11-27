--- src/service/mod.rs.orig
+++ src/service/mod.rs
@@ -1,3 +1,4 @@
+#![recursion_limit = "256"]
 #![expect(refining_impl_trait)]
 
 mod manager;
