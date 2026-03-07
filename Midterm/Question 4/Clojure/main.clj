
;; The BigO of this clojure version of stable-deadup is O(n) due to the fact that we have one loop that goes through
;; the entire list in the program. 
;; This is where majority of our complexity comes from.

;; The trade off for using hashing in clojure for this is clojure objects are always hashable!
;; We take advantage of this in this program by not having to worry about the non hashable types.


(defn stable-dedup [xs]
  (loop [remaining_values xs
         seen_values #{}
         result []]
    (if (empty? remaining_values)
      result
      (let [compare_value (first remaining_values)]
        (if (contains? seen_values compare_value)
          (recur (rest remaining_values) seen_values result)
          (recur (rest remaining_values) (conj seen_values compare_value) (conj result compare_value)))))))




(println "--- Int ---")
(println (stable-dedup [5 4 2 1 6 4 3 2 1 7 6 5 4 3 2 1]))

(println "--- String ---")
(println (stable-dedup ["Hello" "Bonjour" "Hello" "Adios" "Wazup" "Adios"]))