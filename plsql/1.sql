create or replace procedure multiples as
  total numeric := 0;
begin
  for i in 1..999 loop
    if mod(i, 3) = 0 or mod(i, 5) = 0 then
      total := total + i;
    end if;
  end loop;
  dbms_output.put_line(total);
end multiples;
