(defn list-node [atom list-before list-after]
  (list atom list-before list-after))

(defn move-forward [node]
  (if (empty? (nth node 2))
    nil 
    (list-node (first (nth node 2)) 
               (concat (nth node 1) (list (nth node 0)))
               (rest (nth node 2)))))

(defn move-backwards [node]
  (if (empty? (nth node 1))
    nil
    (list-node (last (nth node 1))
               (drop-last (nth node 1))
               (concat (list (nth node 0)) (nth node 2))
               )))





(def node
  (list-node 5 '(1 2 3 4) '(6 7 8 9 10)))
(println (move-forward (move-forward node)))
(println (move-backwards (move-backwards (move-backwards (move-backwards node)))))