(ns imu-in-prac-clj.core
  (:gen-class))

(def empty-store {:events []})

(defn append-event [store event]
  (update store :events conj event))

(defn apply-event [state event]
  (case (:type event)
    :add-item
      (assoc state (:id event) {:text (:text event) :completed false})
    
    :complete-item
      (assoc-in state [(:id event) :completed] true)
    
    :rename-item
      (assoc-in state [(:id event) :text] (:new-text event))
    
    :delete-item
      (dissoc state (:id event))
    
  state))

(defn snapshot-at [store version]
  (reduce apply-event
    {}
      (take version (:events store))
  )
)

(defn history [store step-through]
  (let [total (count (:events store))
    safe-step (max 1 step-through)
    versions (range 0 (+ total 1) safe-step)]
    (map (fn [version] [version (snapshot-at store version)]) versions)
  )
)



(defn concurrency-demo []
  (let [store-atom (atom empty-store)
        done (atom false)
        events [{:type :add-item      :id 1 :text "Buy milk"}
                {:type :add-item      :id 2 :text "Read book"}
                {:type :complete-item :id 1}
                {:type :rename-item   :id 2 :new-text "Read Dune"}
                {:type :delete-item   :id 1}]
        readers (mapv
                  (fn [version]
                    (future
                      (while (not @done)
                        (let [store @store-atom
                              current-version (count (:events store))]
                          (when (>= current-version version)
                            (println "Reader at version" version ":"
                                     (snapshot-at store version)))
                          (Thread/sleep 10)))))
                  (range 1 4))
        writer (future
                 (doseq [event events]
                   (swap! store-atom append-event event)
                   (let [store @store-atom
                         version (count (:events store))]
                     (println "Writer published version" version ":"
                              (snapshot-at store version)))
                   (Thread/sleep 20))
                 (reset! done true)
                 (println "\n--- Full History ---")
                 (doseq [[version state] (history @store-atom 1)]
                   (println "Version" version ":" state)))]
    (deref writer)
    (doseq [r readers] (deref r))))

(defn -main
  [& args]
  (println "\n--- Concurrency Demo ---")
  (concurrency-demo))