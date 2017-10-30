package query;

import java.util.ArrayList;
import java.util.List;

public class Condition {

    String conditionType;
    List<String> conditionList = new ArrayList<>();

    public String getConditionType() {
        return conditionType;
    }

    public void setConditionType(String conditionType) {
        this.conditionType = conditionType;
    }

    public List<String> getConditionList() {
        return conditionList;
    }

    public void setConditionList(List<String> conditionList) {
        this.conditionList = conditionList;
    }

}
