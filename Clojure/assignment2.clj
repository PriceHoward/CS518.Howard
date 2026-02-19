(defn createTreeNode [atom leftList rightList]
 (list atom leftList rightList))

(defn emptyTat? [tat]
 (empty? tat))

(defn member? [tat atom]
 (if (emptyTat? tat)
   false
   (if (= atom (nth tat 0))
     true
     (if (< atom (nth tat 0)) (member? (nth tat 1) atom)
       (member? (nth tat 2) atom)))))

(defn insert [tat atom]
 (if (emptyTat? tat)
   (createTreeNode (atom '() '()))
   (if (= atom (nth tat 0))
     tat
     (if (< atom (nth tat 0))
       (createTreeNode  
                       (nth tat 0)
                       (insert (nth tat 1) atom)
                       (nth tat 2))
       (createTreeNode
                       (nth tat 0)
                       (nth tat 1)
                       (insert (nth tat 2) atom))))))

(defn in-order [tat exp]
  (if (emptyTat? tat)
    '()
    (concat (in-order (nth tat 1) exp)
            (list (exp (nth tat 0)))
            (in-order (nth tat 2) exp))))

(defn pre-order [tat exp]
  (if (emptyTat? tat)
    '()
    (concat (list (exp (nth tat 0)))
            (pre-order (nth tat 1) exp)
            (pre-order (nth tat 2) exp))))
(defn post-order [tat exp]
  (if (emptyTat? tat)
    '()
    (concat (post-order (nth tat 1) exp)
            (post-order (nth tat 2) exp)
            (list (exp (nth tat 0))))))


(defn delete [tat atom]
 (if (emptyTat? tat)
   '()
   (let [current (nth tat 0)
         leftList (nth tat 1)
         rightList(nth tat 2)]
     (cond
           (< atom current) (createTreeNode current (delete leftList atom) rightList)
           (> atom current) (createTreeNode current leftList (delete rightList atom))
           :else
           (cond
             (emptyTat? leftList) rightList
             (emptyTat? rightList) leftList
             :else
             (let [nextN (first (first (in-order rightList list)))]
               (createTreeNode nextN leftList (delete rightList nextN))))))))