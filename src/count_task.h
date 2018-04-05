#ifndef _COUNT_TASK_H_
#define _COUNT_TASK_H_
#include <iostream>
#include <string>
using std::string;

class CCountTask
{
protected:
	string m_strTaskName;  /** 任务的名称 */
	void* m_ptrData;       /** 要执行的任务的具体数据 */
public:
	CCountTask(){}
	CCountTask(string taskName)
	{
		m_strTaskName = taskName;
		m_ptrData = NULL;
	}
	virtual ~CCountTask(){}
	virtual int count() = 0;
	
	void SetData(void* data);    /** 设置任务数据 */
};



#endif
