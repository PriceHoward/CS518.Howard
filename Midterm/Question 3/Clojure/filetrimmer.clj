(require '[clojure.java.io :as io]
         '[clojure.string :as str])

(defn file-manipulation [file-name]
  (let [reader (try (io/reader file-name)
                    (catch java.io.FileNotFoundException e
                      (println "!COULD NOT OPEN INPUT FILE!:" (.getMessage e))
                      (System/exit 1)))]
    (with-open [reader reader]
      (let [writer (try (io/writer (str "corrected_" file-name))
                        (catch java.io.IOException e
                          (println "!COULD NOT OPEN OUTPUT FILE!:" (.getMessage e))
                          (System/exit 1)))]
        (with-open [writer writer]
          (let [lines     (line-seq reader)
                non-blank (filter #(not (str/blank? %)) lines)
                trimmed   (map str/trim non-blank)
                numbered  (map-indexed #(str (+ 1 %1) ": " %2) trimmed)]
            (doseq [line numbered]
              (.write writer (str line "\n")))))))))

(file-manipulation "CoolText.txt")