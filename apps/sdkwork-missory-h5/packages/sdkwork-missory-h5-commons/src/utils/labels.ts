export function relationshipTypeLabel(value: string): string {
  const map: Record<string, string> = {
    family: "家人", friend: "朋友", classmate: "同学", colleague: "同事",
    client: "客户", partner: "合作伙伴", teacher: "老师", student: "学生",
    neighbor: "邻居", spouse: "伴侣", other: "其他",
  };
  return map[value] ?? value;
}

export function memoryTypeLabel(value: string): string {
  const map: Record<string, string> = {
    semantic: "事实", episodic: "经历", temporal: "近况",
    relationship: "关系", preference: "偏好", commitment: "承诺",
  };
  return map[value] ?? value;
}

export function importanceLabel(value: string | undefined): string {
  if (!value) return "普通";
  const map: Record<string, string> = { low: "低", normal: "普通", high: "重要", core: "核心" };
  return map[value] ?? value;
}
