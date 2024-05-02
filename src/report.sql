SELECT p.name, c.description, c.level, c.frequency, date(max(t.done)) 
FROM person p, chore c, assignment a, task t
WHERE p.id = a.person_id AND c.id = a.chore_id AND c.id = t.chore_id
GROUP BY p.id,c.id
