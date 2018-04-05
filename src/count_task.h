#ifndef _COUNT_TASK_H_
#define _COUNT_TASK_H_
#include <iostream>
#include <string>
using std::string;

class CCountTask
{
protected:
	string m_strTaskName;  /** ��������� */
	void* m_ptrData;       /** Ҫִ�е�����ľ������� */
public:
	CCountTask(){}
	CCountTask(string taskName)
	{
		m_strTaskName = taskName;
		m_ptrData = NULL;
	}
	virtual ~CCountTask(){}
	virtual int count() = 0;
	
	void SetData(void* data);    /** ������������ */
};



#endif
