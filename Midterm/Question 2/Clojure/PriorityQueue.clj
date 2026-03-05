(defn new-queue [order]
  {:vector []
   :order order})

(defn push [queue value]
  (let [dataQueue (:vector queue)
        order (:order queue)
        newData (conj dataQueue value)]
    (if order
      (assoc queue :vector (vec (sort newData)))
      (assoc queue :vector (vec (sort > newData))))))

(defn pop-queue [queue]
  (let [dataQueue (:vector queue)
        order (:order queue)]
    (if (empty? dataQueue)
      [nil queue]
      (if order
        [(last dataQueue) (assoc queue :vector (vec (butlast dataQueue)))]
        [(first dataQueue) (assoc queue :vector (vec (rest dataQueue)))]))))

(defn traverse [queue]
  (doseq [queueItem (:vector queue)]
    (println queueItem)))






(println "How would you like your queue?")
(println "0: Highest -> Lowest Priority")
(println "1: Lowest -> Highest Priority")
(let [input (read-line)]
  (def q (atom (cond
                 (= input "0") (new-queue false)
                 (= input "1") (new-queue true)
                 :else (do (println  "Invalid Input, Defaulting to False.")
                                     (new-queue false))))))
(println "--------------------------------------")
(swap! q push 5)
(swap! q push 12)
(swap! q push 1)
(swap! q push 15)
(swap! q push 6)
(swap! q push 7)

(traverse @q)
(println "--------------------------------------")
(let [[val new-q] (pop-queue @q)]
  (println "Popped:" val)
  (reset! q new-q))

(println "--------------------------------------")
(traverse @q)