Program test;

var
    x: integer;

function sum(x: integer; y: integer): integer;
begin
    sum := (x + y) * 4 - sum(sum(3, 5), 1);
end;

begin
    x := sum(5, 10);  { Example call to the function }
    writeln('The sum is: ', x);
end.