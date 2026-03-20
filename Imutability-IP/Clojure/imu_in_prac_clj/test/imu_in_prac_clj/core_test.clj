(ns imu-in-prac-clj.core-test
  (:require [clojure.test :refer :all]
            [imu-in-prac-clj.core :refer :all]))

; Test 1: Old version unchanged
(deftest old-version-unchanged
  (let [s0 empty-store
        s1 (append-event s0 {:type :add-item :id 1 :text "Buy milk"})]

    (is (= 0 (count (snapshot-at s0 0))))

    (is (= 1 (count (snapshot-at s1 1))))))

; Test 2: Non-interference
(deftest non-interference
  (let [s0 empty-store
        s1 (append-event s0 {:type :add-item :id 1 :text "Buy milk"})
        before (count (snapshot-at s1 1))
        _s2 (append-event s1 {:type :add-item :id 2 :text "Read book"})
        after (count (snapshot-at s1 1))]

    (is (= before after))))

; Test 3: Concurrency Sanity
(deftest concurrency-sanity
  (let [store-atom (atom empty-store)
        done (atom false)
        events [{:type :add-item      :id 1 :text "Buy milk"}
                {:type :add-item      :id 2 :text "Read book"}
                {:type :complete-item :id 1}
                {:type :delete-item   :id 1}]

        readers (mapv
                  (fn [version]
                    (future
                      (while (not @done)
                        (let [store @store-atom
                              current-version (count (:events store))]
                          (when (>= current-version version)
                            ; version 1 should always have exactly 1 item
                            (is (= 1 (count (snapshot-at store 1)))))
                          (Thread/sleep 10)))))
                  (range 1 4))

        writer (future
                 (doseq [event events]
                   (swap! store-atom append-event event)
                   (Thread/sleep 20))
                 (reset! done true))]

    (deref writer)
    (doseq [r readers] (deref r))))